using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool PrimeSubOperation(int[] nums)
    {
        int max = nums.Max();
        Span<bool> sieve = stackalloc bool[1 + max];
        sieve.Fill(true);
        sieve[0] = false; sieve[1] = false;
        for (int p = 0; p <= max; p++)
        {
            if (sieve[p])
            {
                for (int i = p * p; i <= max; i += p) { sieve[i] = false; }
            }
        }
        List<int> primes = [];
        for (int i = 0; i <= max; i++)
        {
            if (sieve[i]) { primes.Add(i); }
        }
        int prev = 0;
        foreach (var item in nums)
        {
            if (item <= prev) { return false; }
            int i = primes.BinarySearch(item);
            int end = i < 0 ? (~i) - 1 : i;
            bool found = false;
            for (int p = end; p >= 0; p -= 1)
            {
                if (item - primes[p] > prev)
                {
                    prev = item - primes[p];
                    found = true;
                    break;
                }
            }
            if (!found) { prev = item; }
        }
        return true;
    }
}
