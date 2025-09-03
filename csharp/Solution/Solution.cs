using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindRotateSteps(string ring, string key)
    {
        int rn = ring.Length;
        int kn = key.Length;
        int[,] memo = new int[rn, kn];
        for (int i1 = 0; i1 < rn; i1++)
        {
            for (int i2 = 0; i2 < kn; i2++)
            {
                memo[i1, i2] = -1;
            }
        }
        return Dfs(0, 0);

        int Dfs(int ri, int ki)
        {
            if (ki >= key.Length) { return 0; }
            if (memo[ri, ki] > -1) { return memo[ri, ki]; }
            int res;
            if (ring[ri] == key[ki]) { res = 1 + Dfs(ri, 1 + ki); }
            else
            {
                int step = 0;
                int curr = ri;
                for (; ring[curr] != key[ki]; curr = (1 + curr) % rn)
                {
                    step += 1;
                }
                res = 1 + step + Dfs(curr, 1 + ki);
                step = 0;
                curr = ri;
                for (; ring[curr] != key[ki]; curr = (curr + rn - 1) % rn)
                {
                    step += 1;
                }
                res = int.Min(res, 1 + step + Dfs(curr, 1 + ki));
            }
            memo[ri, ki] = res;
            return res;
        }
    }
}