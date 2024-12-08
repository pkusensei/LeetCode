using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class FizzBuzz
{
    private int n;
    private readonly SemaphoreSlim fizz = new(0);
    private readonly SemaphoreSlim buzz = new(0);
    private readonly SemaphoreSlim fizzbuzz = new(0);
    private readonly SemaphoreSlim number = new(1);

    public FizzBuzz(int n)
    {
        this.n = n;
    }

    // printFizz() outputs "fizz".
    public void Fizz(Action printFizz)
    {
        for (int i = 3; i <= n; i += 3)
        {
            if (i % 5 > 0)
            {
                fizz.Wait();
                printFizz();
                number.Release();
            }
        }
    }

    // printBuzzz() outputs "buzz".
    public void Buzz(Action printBuzz)
    {
        for (int i = 5; i <= n; i += 5)
        {
            if (i % 3 > 0)
            {
                buzz.Wait();
                printBuzz();
                number.Release();
            }
        }
    }

    // printFizzBuzz() outputs "fizzbuzz".
    public void Fizzbuzz(Action printFizzBuzz)
    {
        for (int i = 15; i <= n; i += 15)
        {
            fizzbuzz.Wait();
            printFizzBuzz();
            number.Release();
        }
    }

    // printNumber(x) outputs "x", where x is an integer.
    public void Number(Action<int> printNumber)
    {
        for (int i = 1; i <= n; i++)
        {
            number.Wait();
            if (i % 15 == 0) { fizzbuzz.Release(); }
            else if (i % 3 == 0) { fizz.Release(); }
            else if (i % 5 == 0) { buzz.Release(); }
            else { printNumber(i); number.Release(); }
        }
    }
}
