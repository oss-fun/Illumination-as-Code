# variables that can be overriden
variable "hostname" { default = "simple" }
variable "domain" { default = "example.com" }
variable "memoryMB" { default = 1024 * 1 }
variable "cpu" { default = 1 }
# 下記terraform {}内は追記by西
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
  name   = "${var.hostname}-os_image"
  pool   = "default"
  source = "https://cloud-images.ubuntu.com/bionic/current/bionic-server-cloudimg-amd64.img"
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
  name   = var.hostname
  memory = var.memoryMB
  vcpu   = var.cpu
  disk {
    volume_id = libvirt_volume.os_image.id
  }
  network_interface {
    network_name   = "default"
    wait_for_lease = true
    addresses      = ["192.168.122.209"]
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
    source = "hello.txt"
    destination = "hello.txt"
    connection {
      type        = "ssh"
      user        = "ubuntu"
      host_key = file("id_rsa")
      private_key = file("id_rsa.pub")
      host        = "192.168.122.209"
    }
  }
}

output "ips" {
  # show IP, run 'terraform refresh' if not populated
  value = libvirt_domain.domain-ubuntu.*.network_interface.0
}
