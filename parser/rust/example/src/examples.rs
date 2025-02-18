
pub const MMAP_ANONYMOUS: &str = "1730805691.281081 mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f1f1cf41000 <0.000021>";
pub const OPENAT: &str = "1730805691.259985 openat(AT_FDCWD</home/veldrane>, \"/etc/ld.so.cache\", O_RDONLY|O_CLOEXEC) = 3</etc/ld.so.cache> <0.000028>>";
pub const ACCEPT: &str = "1730805682.451869 accept(3<socket:[21159]>, {sa_family=AF_INET, sin_port=htons(45986), sin_addr=inet_addr(\"10.1.8.52\")}, [128 => 16]) = 5<socket:[21170]> <0.000121>";
pub const CONNECT: &str = "1730805604.253901 connect(4<socket:[21152]>, {sa_family=AF_UNIX, sun_path=\"/var/run/nscd/socket\"}, 110) = -1 ENOENT (No such file or directory) <0.000135>";
pub const BIND: &str = "1730805604.315960 bind(3<socket:[21159]>, {sa_family=AF_INET, sin_port=htons(10022), sin_addr=inet_addr(\"0.0.0.0\")}, 16) = 0 <0.000072>";
pub const MMAP_FILE: &str = "1708596064.182267 mmap(0x7f0bcf06b000, 204800, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3</usr/lib/x86_64-linux-gnu/libcrypto.so.1.1>, 0x2b5000) = 0x7f0bcf06b000 <0.000017>";
pub const SOCKET_UNIX: &str = "1732700675.480130 socket(AF_UNIX, SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK, 0) = 7<\x73\x6f\x63\x6b\x65\x74\x3a\x5b\x36\x31\x39\x35\x36\x39\x5d> <0.000007>";
pub const SOCKET_INET: &str = "1732700675.481687 socket(AF_INET, SOCK_DGRAM|SOCK_CLOEXEC|SOCK_NONBLOCK, IPPROTO_IP) = 7<\x73\x6f\x63\x6b\x65\x74\x3a\x5b\x36\x31\x39\x35\x37\x30\x5d> <0.000009>";
pub const LISTEN: &str = "1730805604.319413 listen(4<socket:[21160]>, 128) = 0 <0.000064>";