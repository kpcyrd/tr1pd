
    # build image
    docker build -t kpcyrd/tr1pd .
    # start tr1pd
    docker run --rm --init --name tr1pd kpcyrd/tr1pd
    # attach tr1pctl
    docker run -it --rm --init --volumes-from tr1pd kpcyrd/tr1pd tr1pctl head

