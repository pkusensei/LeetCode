// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxDepth(Node root)
    {
        if (root is null) { return 0; }
        var res = 1;
        foreach (var node in root.children)
        {
            res = Math.Max(res, 1 + MaxDepth(node));
        }
        return res;
    }
}
