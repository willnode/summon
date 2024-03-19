NGINX_VERSION=1.22.1
PCRE2_VERSION=10.39
ZLIB_VERSION=1.3.1
OPENSSL_VERSION=3.2.0
# Build the Docker image
.PHONY: build
build:
	zig build
# cp

.PHONY: test
test:
	cd ./nginx-install && ./sbin/nginx -V
	cd ./nginx-install && ./sbin/nginx

nginx-install: nginx-${NGINX_VERSION} openssl-${OPENSSL_VERSION} pcre2-${PCRE2_VERSION} zlib-${ZLIB_VERSION}
	cd nginx-${NGINX_VERSION} && ./configure \
	--prefix=../nginx-install \
	--with-pcre=../pcre2-${PCRE2_VERSION} \
	--with-openssl=../openssl-${PCRE2_VERSION} \
	--with-zlib=../zlib-${ZLIB_VERSION} \
	&& make install

nginx-${NGINX_VERSION}:
	wget http://nginx.org/download/nginx-${NGINX_VERSION}.tar.gz
	tar -xf nginx-${NGINX_VERSION}.tar.gz

openssl-${OPENSSL_VERSION}:
	wget https://github.com/openssl/openssl/releases/download/openssl-${OPENSSL_VERSION}/openssl-${OPENSSL_VERSION}.tar.gz
	tar -xf openssl-${OPENSSL_VERSION}.tar.gz

pcre2-${PCRE2_VERSION}:
	wget https://github.com/PCRE2Project/pcre2/releases/download/pcre2-${PCRE2_VERSION}/pcre2-${PCRE2_VERSION}.tar.gz
	tar -xf pcre2-${PCRE2_VERSION}.tar.gz

zlib-${ZLIB_VERSION}:
	wget https://www.zlib.net/zlib-${ZLIB_VERSION}.tar.gz
	tar -xf zlib-${ZLIB_VERSION}.tar.gz