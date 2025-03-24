using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] ClosestPrimes(int left, int right)
    {
        int[] res = [-1, -1];
        int[] sieve = Sieve();
        int diff = right;
        for (int i = 0; i < sieve.Length - 1; i++)
        {
            int d = sieve[1 + i] - sieve[i];
            if (left <= sieve[i] && d < diff)
            {
                diff = d;
                res = [sieve[i], sieve[1 + i]];
                if (diff == 2) { break; }
            }
        }
        return res;

        int[] Sieve()
        {
            var sieve = new bool[1 + right];
            sieve[0] = true;
            sieve[1] = true;
            for (int p = 2; p <= Math.Sqrt(right); p++)
            {
                if (!sieve[p]) // p is prime
                {
                    for (int v = p * p; v <= right; v += p)
                    {
                        sieve[v] = true;
                    }
                }
            }
            return [.. sieve.Select((v, i) => (i, v)).Where(p => !p.v).Select(p => p.i)];
        }
    }
}
