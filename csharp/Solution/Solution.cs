using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class ZeroEvenOdd
{
    private int n;
    bool zero = true;
    int curr = 0;

    public ZeroEvenOdd(int n)
    {
        this.n = n;
    }

    // printNumber(x) outputs "x", where x is an integer.
    public void Zero(Action<int> printNumber)
    {
        while (curr < n)
        {
            SpinWait.SpinUntil(() => zero);
            printNumber(0);
            curr += 1;
            zero = false;
        }
    }

    public void Even(Action<int> printNumber)
    {
        while (curr <= n)
        {
            SpinWait.SpinUntil(() => !zero && (curr & 1) == 0);
            if (curr > n) { break; }
            printNumber(curr);
            if (curr >= n)
            {
                curr += 1;
                break;
            }
            zero = true;
        }
    }

    public void Odd(Action<int> printNumber)
    {
        while (curr <= n)
        {
            SpinWait.SpinUntil(() => !zero && (curr & 1) == 1);
            if (curr > n) { break; }
            printNumber(curr);
            if (curr >= n)
            {
                curr += 1;
                break;
            }
            zero = true;
        }
    }
}
