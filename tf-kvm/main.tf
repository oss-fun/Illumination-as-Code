variable "hostname" { default = "tf-vm" }
variable "domain" { default = "illumination-as-code.com" }
variable "machine_num" { default = 3 }
variable "cpu" {
  type    = tuple([number, number, number])
  default = [1, 1, 1]
}

terraform {
  required_version = ">= 0.13"
  required_providers {
    libvirt = {
      version = ">= 0.7.6"
      source  = "dmacvicar/libvirt"
    }
  }
}

provider "libvirt" {
  uri = "qemu:///system"
}

resource "libvirt_volume" "os_image" {
  count = var.machine_num
  name   = "${var.hostname}${count.index}-vm-os_image"
  pool   = "default"
  source = "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img"
  format = "qcow2"
}

resource "libvirt_cloudinit_disk" "commoninit" {
  count = var.machine_num
  name           = "${var.hostname}${count.index}-init.iso"
  pool           = "default"
  user_data      = <<-EOF
  #cloud-config
  hostname: ${var.hostname}
  fqdn: ${var.hostname}.${var.domain}
  manage_etc_hosts: true
  users:
    - name: ubuntu
      sudo: ALL=(ALL) NOPASSWD:ALL
      groups: users, admin
      home: /home/ubuntu
      shell: /bin/bash
      lock_passwd: false
      ssh-authorized-keys:
        - ${file("id_ed25519.pub")}
  # only cert auth via ssh (console access can still login)
  ssh_pwauth: false
  disable_root: false
  chpasswd:
    list: |
       ubuntu:linux
    expire: False
  packages:
   - qemu-guest-agent
  EOF
  network_config = <<-EOF
  version: 2
  ethernets:
    ens3:
      addresses:
      - "192.168.${count.index+2}.22/24"
      gateway4: 192.168.${count.index+2}.1
      nameservers:
        addresses:
          - 8.8.8.8
          - 8.8.4.4
  EOF
}

resource "libvirt_domain" "vm" {
  count = var.machine_num
  name   = "${var.hostname}-${count.index+2}"
  memory = 2048
  vcpu   = var.cpu[count.index]

  disk {
    volume_id = element(libvirt_volume.os_image.*.id,count.index)
  }

  network_interface {
    bridge = "br${count.index+2}"
    addresses = ["192.168.${count.index+2}.22"]
  }

  cloudinit = element(libvirt_cloudinit_disk.commoninit.*.id,count.index)

  console {
    type        = "pty"
    target_port = "0"
    target_type = "serial"
  }

  graphics {
    type        = "spice"
    listen_type = "address"
    autoport    = "true"
  }
  
  provisioner "file" {
    source      = "compute/compute_pi"
    destination = "/tmp/compute_pi"
    connection {
      type        = "ssh"
      user        = "ubuntu"
      private_key = file("id_ed25519")
      host        = format("192.168.%d.22", count.index + 2)
    }
  }

  provisioner "file" {
    source      = format("compute/pi%d.service", count.index + 1)
    destination = format("/tmp/pi%d.service", count.index + 1)
    connection {
      type        = "ssh"
      user        = "ubuntu"
      private_key = file("id_ed25519")
      host        = format("192.168.%d.22", count.index + 2)
    }
  }
  provisioner "remote-exec" {
    inline = [
      "${format("sudo mv /tmp/pi%d.service /etc/systemd/system/pi%d.service", count.index + 1, count.index + 1)}",
      "sudo chmod 777 /tmp/compute_pi",
      "sudo systemctl daemon-reload",
      "${format("sudo systemctl start pi%d.service", count.index + 1)}",
    ]
    connection {
      type        = "ssh"
    user        = "ubuntu"
      private_key = file("id_ed25519")
      host        = format("192.168.%d.22", count.index + 2)
    }
  }
}
