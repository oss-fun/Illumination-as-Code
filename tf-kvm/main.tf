variable "hostname" { default = ["tf-r","tf-g","tf-b"] }
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
  name   = "${var.hostname[count.index]}-vm-os_image"
  pool   = "default"
  source = "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img"
  format = "qcow2"
}

resource "libvirt_cloudinit_disk" "commoninit" {
  count = var.machine_num
  name           = "${var.hostname[count.index]}-common-init.iso"
  pool           = "default"
  user_data      = <<-EOF
  #cloud-config
  hostname: ${var.hostname[count.index]}
  fqdn: ${var.hostname[count.index]}.${var.domain}
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
  package_update: false
  package_upgrade: false
  packages:
   - qemu-guest-agent
  EOF
  network_config = <<-EOF
  version: 2
  ethernets:
    ens3:
      addresses:
      - "192.168.22.${count.index+2}/24"
      gateway4: 192.168.22.1
      nameservers:
        addresses:
          - 8.8.8.8
          - 8.8.4.4
  EOF
}

resource "libvirt_domain" "vm" {
  count = var.machine_num
  name   = var.hostname[count.index]
  memory = 2048
  vcpu   = var.cpu[count.index]

  disk {
    volume_id = element(libvirt_volume.os_image.*.id,count.index)
  }

  network_interface {
    bridge = "br${count.index+2}"
    addresses = ["192.168.22.${count.index+2}"]
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
}

