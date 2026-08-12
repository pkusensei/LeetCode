using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class FizzBuzz
{
    private readonly int n;
    private int num = 1;
    private readonly object _lock = new();

    public FizzBuzz(int n)
    {
        this.n = n;
    }

    // printFizz() outputs "fizz".
    public void Fizz(Action printFizz)
    {
        while (num <= n)
        {
            lock (_lock)
            {
                while (num <= n && !(num % 3 == 0 && num % 5 > 0))
                {
                    Monitor.Wait(_lock);
                }
                if (num <= n)
                {
                    printFizz();
                    num += 1;
                }
                Monitor.PulseAll(_lock);
            }
        }
    }

    // printBuzzz() outputs "buzz".
    public void Buzz(Action printBuzz)
    {
        while (num <= n)
        {
            lock (_lock)
            {
                while (num <= n && !(num % 5 == 0 && num % 3 > 0))
                {
                    Monitor.Wait(_lock);
                }
                if (num <= n)
                {
                    printBuzz();
                    num += 1;
                }
                Monitor.PulseAll(_lock);
            }
        }
    }

    // printFizzBuzz() outputs "fizzbuzz".
    public void Fizzbuzz(Action printFizzBuzz)
    {
        while (num <= n)
        {
            lock (_lock)
            {
                while (num <= n && num % 15 > 0)
                {
                    Monitor.Wait(_lock);
                }
                if (num <= n)
                {
                    printFizzBuzz();
                    num += 1;
                }
                Monitor.PulseAll(_lock);
            }
        }
    }

    // printNumber(x) outputs "x", where x is an integer.
    public void Number(Action<int> printNumber)
    {
        while (num <= n)
        {
            lock (_lock)
            {
                while (num <= n && !(num % 3 > 0 && num % 5 > 0))
                {
                    Monitor.Wait(_lock);
                }
                if (num <= n)
                {
                    printNumber(num);
                    num += 1;
                }
                Monitor.PulseAll(_lock);
            }
        }
    }
}