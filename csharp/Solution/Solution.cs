using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NthSuperUglyNumber(int n, int[] primes)
    {
        HashSet<int> seen = [];
        PriorityQueue<int, int> pq = new();
        pq.Enqueue(1, 1);
        while (pq.TryDequeue(out int num, out _))
        {
            if (seen.Add(num))
            {
                n -= 1;
                if (n == 0) { return num; }
                try
                {
                    checked
                    {
                        foreach (var p in primes)
                        {
                            pq.Enqueue(p * num, p * num);
                        }
                    }
                }
                catch (OverflowException) { }
            }
        }
        return -1;
    }

    public int WithDp(int n, int[] primes)
    {
        if (n <= 1) { return 1; }
        int[] ugly = new int[n];
        ugly[0] = 1;
        // Each ptr corresponds to one prime.
        // ptrs[i] is the index in `ugly` array that primes[i] should multiply
        int[] ptrs = new int[primes.Length];
        // Each prime with multiplication produces these candidates
        // init as prime*ugly[0]
        long[] values = [.. primes];
        for (int idx = 1; idx < n; idx++)
        {
            long curr_min = values.Min();
            ugly[idx] = (int)curr_min;
            for (int i = 0; i < primes.Length; i++)
            {
                // This candidate has be "used"
                if (values[i] == curr_min)
                {
                    // Its ptr has to be updated to next in ugly
                    ptrs[i] += 1;
                    // update new candidate
                    values[i] = (long)ugly[ptrs[i]] * primes[i];
                }
            }
        }
        return ugly[n - 1];
    }
}