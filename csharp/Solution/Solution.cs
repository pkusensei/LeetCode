using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<TreeNode> AllPossibleFBT(int n)
    {
        if (n == 1)
        {
            return [new(0)];
        }
        if (n == 3)
        {
            return [new(0, new(0), new(0))];
        }
        List<TreeNode> res = [];
        for (int i = 1; i < n - 1; i += 2)
        {
            var left = AllPossibleFBT(i);
            var right = AllPossibleFBT(n - 1 - i);
            foreach (var a in left)
            {
                foreach (var b in right)
                {
                    res.Add(new(0, a, b));
                }
            }
        }
        return res;
    }
}
