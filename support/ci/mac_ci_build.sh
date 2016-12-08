pushd ../../components/hab/mac
sudo ./mac-build.sh
scp ./results/ private-depot:deploy/mac/
ssh private-depot ./deploy/unstable_mac_hab.sh
popd
