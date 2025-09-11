using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindIntegers(int n)
    {
        int bit_width = 1 + int.Log2(n);
        int[,,] memo = new int[bit_width, 2, 2];
        for (int i1 = 0; i1 < bit_width; i1++)
        {
            for (int i2 = 0; i2 < 2; i2++)
            {
                for (int i3 = 0; i3 < 2; i3++)
                {
                    memo[i1, i2, i3] = -1;
                }
            }
        }
        return Dfs(bit_width - 1, true, 0);

        int Dfs(int idx, bool tight, int prev)
        {
            if (idx < 0) { return 1; }
            int bi = tight ? 1 : 0;
            if (memo[idx, bi, prev] > -1) { return memo[idx, bi, prev]; }
            int upper = tight ? ((n >> idx) & 1) : 1;
            int res = 0;
            for (int bit = 0; bit <= upper; bit++)
            {
                if (prev == 1 && bit == 1) { continue; }
                bool ntight = tight && bit == upper;
                res += Dfs(idx - 1, ntight, bit);
            }
            memo[idx, bi, prev] = res;
            return res;
        }
    }

    public int DigitDpWithFib(int n)
    {
        Span<int> fib = stackalloc int[32];
        fib[0] = 1; // bit_width=0; zero numbers
        fib[1] = 2; // bit_width=1; 0 and 1
        for (int i = 2; i < 32; i++)
        {
            // [i]=0, add to all fib[i-1] numbers
            // [i]=1, [i-1] has to be 0, add "10" to all fib[i-2]
            fib[i] = fib[i - 1] + fib[i - 2];
        }
        int res = 0;
        int prev = 0;
        for (int i = 31; i >= 0; i -= 1)
        {
            if (((n >> i) & 1) == 1)
            {
                // this bit is 1, if we put 0 here
                // all fib[i] numbers are valid
                // If this bit is 0, continue the same prefix as `n`
                res += fib[i];
                if (prev == 1) { return res; } // consecutive 1's
                prev = 1;
            }
            else
            {
                prev = 0;
            }
        }
        return 1 + res; // +1 to count `n` in
    }
}