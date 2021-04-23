template <typename T>
struct Foo {
    template <typename> using SecondAlias = Foo<T>;
    SecondAlias<int> member;
};
