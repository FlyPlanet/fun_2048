# fun_2048

这是一个使用Rust写成的练手项目，核心内容为一个2048游戏

## main分支

使用了 qmetaobject-rs crate 提供的qml支持，写了一个gui程序，你可以在上面游玩2048。展示了 qmetaobject-rs 的使用逻辑。

## fun分支

这个算是为了满足自己的一个小脑洞，想看看如果随机操作游戏的话，分数的分布是什么样子。

使用 rayon crate 提供的并行计算能力进行大量模拟，tqdm crate 来呈现进度条，使用 jupyter notebook 来将数据绘成表格