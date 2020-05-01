def sort(x, up):
    """
    リストxの要素をupで指定された向きにソートする。
    xの要素数は2のべきうじょうでなければならない
    """
    if len(x) <= 1:
        return x

    else:
        # リストの前半は昇順、後半は降順でソートする
        mid_point = len(x) // 2
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)

        x1 = first + second

        return _sub_sort(x1, up)

def _sub_sort(x, up):
    """
    バイトニックにソートされたリストxの前半と後半を、upで指定された向きに比較交換し、
    前半と後半それぞれについて再帰的にサブソートを適用する
    """
    if len(x) == 1:
        return x

    else:
        # 要素数nのバイトニック列の要素をn/2要素おきに比較して
        # upで指定された順序(昇順または降順)になるよう交換する
        _compare_and_swap(x, up)

        # データ列を半分に分割し、それぞれに対して_sub_sortを繰り返す
        mid_point = len(x) // 2
        first = _sub_sort(x[:mid_point], up)
        second = _sub_sort(x[mid_point:], up)

	# 2分割したデータ列を1つに結合する
        return first + second

def _compare_and_swap(x, up):
    """
    要素数nのバイトニック列の要素をn/2要素おきに比較して、upで指定された順序になるよう
    交換する
    """

    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            x[i], x[mid_point + i] = x[mid_point + i], x[i]
