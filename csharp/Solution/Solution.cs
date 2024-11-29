using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class FooBar
{
    private int n;
    int flag = 0;

    public FooBar(int n)
    {
        this.n = n;
    }

    public void Foo(Action printFoo)
    {

        for (int i = 0; i < n; i++)
        {
            while (0 != Interlocked.CompareExchange(ref flag, 0, 0)) { Thread.Sleep(1); }
            // printFoo() outputs "foo". Do not change or remove this line.
            printFoo();
            Interlocked.Increment(ref flag);
        }
    }

    public void Bar(Action printBar)
    {

        for (int i = 0; i < n; i++)
        {
            while (1 != Interlocked.CompareExchange(ref flag, 1, 1)) { Thread.Sleep(1); }
            // printBar() outputs "bar". Do not change or remove this line.
            printBar();
            Interlocked.Decrement(ref flag);
        }
    }
}
