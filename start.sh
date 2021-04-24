arch=`uname -m`
case $arch in aarch64)
  wget https://raw.githubusercontent.com/Lewizh11/client-api/main/builds/aarch-android/aarch && chmod 777 aarch && ./aarch
;;
armv7l)
  wget https://raw.githubusercontent.com/Lewizh11/client-api/main/builds/arm-android/armv && chmod 777 armv && ./armv
;;
esac
