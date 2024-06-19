# variables that can be overriden
variable "hostname" { default = "terraform" }
variable "domain" { default = "illumination-as-code.com" }
variable "memoryMB" { default = 1024 * 1 }
variable "machine_num" { default = 3 }
variable "cpu" {
  type    = tuple([number, number, number])
  default = [1, 1, 1]
}
variable "ip" { default = "192.168.122.13" }
terraform {
  required_version = ">= 0.13"
  required_providers {
    libvirt = {
      version = "> 0.6.2"
      source  = "dmacvicar/libvirt"
    }
  }
}
# instance the provider
provider "libvirt" {
  uri = "qemu:///system"
}

# fetch the latest ubuntu release image from their mirrors
resource "libvirt_volume" "os_image" {
  count  = var.machine_num
  name   = format("${var.hostname}-vm%02d-os_image", count.index + 1)
  pool   = "default"
  source = "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img"
  format = "qcow2"
}

# Use CloudInit ISO to add ssh-key to the instance
resource "libvirt_cloudinit_disk" "commoninit" {
  name           = "${var.hostname}-commoninit.iso"
  pool           = "default"
  user_data      = data.template_file.user_data.rendered
  network_config = data.template_file.network_config.rendered
}


data "template_file" "user_data" {
  template = file("${path.module}/cloud_init.cfg")
  vars = {
    hostname = var.hostname
    fqdn     = "${var.hostname}.${var.domain}"
  }
}

data "template_file" "network_config" {
  template = file("${path.module}/network_config_dhcp.cfg")
}


# Create the machine
resource "libvirt_domain" "domain-ubuntu" {
  count  = var.machine_num
  name   = format("${var.hostname}%02d", count.index + 1)
  memory = var.memoryMB
  vcpu   = var.cpu[count.index]

  disk {
    volume_id = libvirt_volume.os_image[count.index].id
  }
  network_interface {
    network_name   = "default"
    bridge = "${format("bridge-br%d", count.index + 3)}"
    wait_for_lease = true
    addresses      = ["${format("${var.ip}%d", count.index)}"]
  }

  cloudinit = libvirt_cloudinit_disk.commoninit.id

  # IMPORTANT
  # Ubuntu can hang is a isa-serial is not present at boot time.
  # If you find your CPU 100% and never is available this is why
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
    source      = "compute"
    destination = "compute"
    connection {
      type        = "ssh"
      user        = "ubuntu"
      private_key = file("id_rsa")
      host        = format("${var.ip}%d", count.index)
    }
  }

  provisioner "remote-exec" {
    connection {
      type        = "ssh"
      user        = "ubuntu"
      private_key = file("id_rsa")
      host        = format("${var.ip}%d", count.index)
    }
    inline = [
      "cd compute",
      "chmod 777 script.sh",
      "chmod 777 compute_pi",
      "./script.sh",
    ]
  }
}


output "ips" {
  # show IP, run 'terraform refresh' if not populated
  value = libvirt_domain.domain-ubuntu.*.network_interface.0.addresses
}
