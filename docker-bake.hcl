variable "RUST_VERSION" {
  default = "1"
}

variable "XX_VERSION" {
  default = "master"
}

variable "RELEASE_TAG" {
  default = null
}

variable "REPO" {
  default = "tamasfe/taplo"
}

variable "PUSH" {
  default = false
}

variable "platforms" {
  default = [
    "linux/i386",
    "linux/amd64",
    "linux/arm64",
    "linux/riscv64",
    "linux/arm/v7",
  ]
}

target "_platforms" {
  platforms = platforms
}

group "default" {
  targets = ["binary", "oci", "oci-shell"]
}

target "binary" {
  inherits = ["alpine"]
  output   = ["type=local,dest=target/alpine"]
  target   = "binary"
}

target "oci" {
  inherits = ["alpine"]
  output   = ["type=image,push=${PUSH}"]
  target   = "oci"
  tags     = flatten(["${REPO}:latest", RELEASE_TAG != null ? ["${REPO}:${RELEASE_TAG}"] : []])
}

target "oci-shell" {
  inherits = ["alpine"]
  output   = ["type=image,push=${PUSH}"]
  target   = "oci-shell"
  tags     = flatten(["${REPO}:latest-shell", RELEASE_TAG != null ? ["${REPO}:${RELEASE_TAG}-shell"] : []])
}

target "alpine" {
  context    = "."
  platforms  = platforms
  pull       = true
  dockerfile = "docker/alpine/Dockerfile"
  args = {
    RUST_VERSION = RUST_VERSION
    XX_VERSION   = XX_VERSION
  }
}
