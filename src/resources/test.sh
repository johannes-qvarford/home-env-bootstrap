echo we are going to do some tests to make sure that io works, OK?
echo we start by asking for su permissions by "sudo cat /etc/hosts".
echo -n "> "
read first

sudo cat /etc/hosts
echo we will now read something from the terminal and print it back to you.
echo -n "> "
read second
echo "you wrote: $second"