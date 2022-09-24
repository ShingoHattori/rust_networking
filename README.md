# rust_networking

rustでネットワークプログラミングをやってみる．

# 最終目標

グローバルIPを持つホストをハブとして，Tagged VLANを捌けるL2 VPNもどきを作成する．


# ディレクトリ構成

### show_intmacaddr

引数に与えられたインターフェースのmac addrを表示する．簡単なイーサネットブリッジを作成する時に，macフレームの書き換えが必要になり，ここでmac addrの取得をする必要があるから，その練習．
