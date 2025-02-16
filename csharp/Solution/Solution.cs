using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] ConstructDistancedSequence(int n)
    {
        var res = new int[2 * n - 1];
        var seen = new bool[1 + n];
        Backtrack(0);
        return res;

        bool Backtrack(int idx)
        {
            if (idx >= res.Length) { return true; }
            if (res[idx] > 0) { return Backtrack(1 + idx); }
            else
            {
                for (int num = n; num > 0; num -= 1)
                {
                    if (seen[num]) { continue; }
                    seen[num] = true;
                    res[idx] = num;
                    if (num == 1)
                    {
                        if (Backtrack(1 + idx)) { return true; }
                    }
                    else if (idx + num < res.Length && res[idx + num] == 0)
                    {
                        res[idx + num] = num;
                        if (Backtrack(1 + idx)) { return true; }
                        res[idx + num] = 0;
                    }
                    res[idx] = 0;
                    seen[num] = false;
                }
            }
            return false;
        }
    }
}
