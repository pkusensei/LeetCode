using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int PunishmentNumber(int n)
    {
        int res = 0;
        for (int i = 1; i <= n; i++)
        {
            if (Backtrack(i, i * i)) { res += i * i; }
        }
        return res;

        static bool Backtrack(int target, int val)
        {
            if (target == val) { return true; }
            if (target < 0 || val < target) { return false; }
            return Backtrack(target - val % 10, val / 10)
                   || Backtrack(target - val % 100, val / 100)
                   || Backtrack(target - val % 1000, val / 1000);
        }
    }
}
