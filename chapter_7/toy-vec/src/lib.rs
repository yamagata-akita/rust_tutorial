pub struct ToyVec<T> {
    elements: Box<[T]>, // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    len: usize,         // ベクタの長さ(現在の要素数)

    // Box<[T]>型はボックス化されたスライス型で、実データをヒープ領域に置く
    // Box<[T]>の値は一度作ったらサイズが変更できない。
}

// 構造体や列挙型では参照型のフィールドをもたせられる。
// その際はライフタイム指定しをつける必要がある
pub struct Iter<'vec, T> {
    // ライフタイムの指定により、このイテレータ自身またはnext()で得た&'vec T型の値が
    // 生存している間は、ToyVecは変更できない
    elements: &'vec Box<[T]>,   // ToyVec構造体のelementsを指す不変の参照
    len: usize,                 // ToyVecの長さ
    pos: usize,                 // 次に返す要素のインデックス
}

// implブロック内に関連関数やメソッドを定義していく。トレイト境界としてDefaultを設定する
impl<T: Default> ToyVec<T> {

    // newはキャパシティ(容量)が0のToyVecを作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // T型の値がsize個格納できるBox<[T]>を返す
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)             // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>()    // Vec<T>に収集してから
            .into_boxed_slice()     // Box<[T]>に変換する
    }

    // ベクタの長さを返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        self.elements.len()     // elementsの要素数(len)がToyVecのキャパシティになる
    }

    pub fn push(&mut self, element: T) {
        // 第一引数に&mut selfをとるため、ToyVec構造体の内容を変更することがわかる
        // 第二引数はT型のため、所有権がこのメソッドへムーブすることがわかる
        // (そして、構造体へムーブするだろうと想像できる)

        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;      // 要素を格納する(所有権がムーブする)
        self.len += 1;
    }


    pub fn get(&self, index: usize) -> Option<&T> {
        // Option<&T>を返すため、selfが所有する不変の参照を返すことがわかる

        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    // self, default, 戻り値のライフタイムを同じにする
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        /*
        match self.get(index) {
            Some(v) => v,           // selfが所有する値(elements)からの借用なので'a
            None => default,        // defaultのライフタイムは'b。戻り値のライフタイムと合わない
        }
        */
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        // 戻り値が参照ではない。所有権ごと返す
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // let elem = self.elements[self.len];
            // →エラー(&mut self)経由では、それが所有する値の所有権を奪えない
            // 変わりの値となら交換できる(ここではデフォルト値を使用)
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            // std::mem::replaceは第一引数の場所にある値を第二引数の値で置き換え、
            // 置き換え前の値を返す
            Some(elem)
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elementsを置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら、要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,       // Iter構造体の定義より、ライフタイムは'vecになる
            len: self.len,
            pos: 0,
        }
    }
}

// Iter<T>にIteratorトレイトを実装する
impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型(トレイトに関連づいた型)で、このイテレータがいてレートする要素の型を指定する
    type Item = &'vec T;

    // nextメソッドは次の要素を返す
    // 要素があるなら不変の参照(&T)をSomeに包んで返し、ないときはNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}
