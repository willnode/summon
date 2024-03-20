NGINX_VERSION=1.22.1
PCRE2_VERSION=10.39
ZLIB_VERSION=1.3.1
OPENSSL_VERSION=3.2.0

.PHONY: build
build:
	PCRE2_VERSION=${PCRE2_VERSION} \
	OPENSSL_VERSION=${OPENSSL_VERSION} \
	NGX_VERSION=${NGINX_VERSION} \
	ZLIB_VERSION=${ZLIB_VERSION} \
	cargo build
	cp target/debug/libsummonapp.so nginx-install/modules/libsummonapp.so

.PHONY: run
run: nginx-install
	cp ./test/nginx.conf ./nginx-install/conf/nginx.conf
	cd ./nginx-install && export RUST_BACKTRACE=1 && ./sbin/nginx

nginx-install: nginx-${NGINX_VERSION} openssl-${OPENSSL_VERSION} pcre2-${PCRE2_VERSION} zlib-${ZLIB_VERSION}
	cd nginx-${NGINX_VERSION} && ./configure \
	--prefix=../nginx-install \
	--with-pcre=../pcre2-${PCRE2_VERSION} \
	--with-openssl=../openssl-${OPENSSL_VERSION} \
	--with-zlib=../zlib-${ZLIB_VERSION} \
	--with-compat \
	--with-http_addition_module \
	--with-http_auth_request_module \
	--with-http_flv_module \
	--with-http_gunzip_module \
	--with-http_gzip_static_module \
	--with-http_random_index_module \
	--with-http_realip_module \
	--with-http_secure_link_module \
	--with-http_slice_module \
	--with-http_slice_module \
	--with-http_ssl_module \
	--with-http_stub_status_module \
	--with-http_sub_module \
	--with-http_v2_module \
	--with-stream_realip_module \
	--with-stream_ssl_module \
	--with-stream_ssl_preread_module \
	--with-stream \
	--with-threads \
	--with-file-aio \
	"--with-cc-opt=-g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC" \
	"--with-ld-opt=-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie" \
	&& make install
	mkdir -p nginx-install/modules
	rm -rf nginx-install/html
	ln -s $(shell pwd)/test/html ./nginx-install/html

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
