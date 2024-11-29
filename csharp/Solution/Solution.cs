using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Foo
{
    int count = 0;

    public Foo() { }

    public void First(Action printFirst)
    {
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        Interlocked.Increment(ref count);
    }

    public void Second(Action printSecond)
    {
        while (1 != Interlocked.CompareExchange(ref count, 1, 1)) { Thread.Sleep(10); }
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        Interlocked.Increment(ref count);
    }

    public void Third(Action printThird)
    {
        while (2 != Interlocked.CompareExchange(ref count, 0, 2)) { Thread.Sleep(10); }
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
    }
}
