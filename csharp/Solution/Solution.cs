using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SmallestRepunitDivByK(int k)
    {
        bool[] seen = new bool[1 + k];
        int curr = 1;
        int len = 1;
        while (true)
        {
            if (curr % k == 0) { return len; }
            if (seen[curr]) { return -1; }
            seen[curr] = true;
            curr = (10 * curr + 1) % k;
            len += 1;
        }
    }
}

