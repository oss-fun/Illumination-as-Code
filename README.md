# Illumination-as-Code
すめし、にしのOSCデモ



terraform について
  tf-kvm内でterraform apply -auto-approve -var 'cpu=["2", "2", "2"]'で実行可能
  上記のcpuの数値を変えてapplyすることで自動的に必要最低限のdestroyが自動で起きる。
  その場合IPアドレスを変更する必要がない。
  また、terraform destroy -target=libvirt_domain.domain-ubuntuと指定してdestroyした場合でも
  問題なく動作する。
