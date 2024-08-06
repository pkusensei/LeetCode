public class TreeNode
{
    public int val;
    public TreeNode left;
    public TreeNode right;
    public TreeNode(int val = 0, TreeNode left = null, TreeNode right = null)
    {
        this.val = val;
        this.left = left;
        this.right = right;
    }

    public List<int> InorderTrav()
    {
        List<int> res = [];
        if (left is not null)
        {
            res.AddRange(left.InorderTrav());
        }
        res.Add(val);
        if (right is not null)
        {
            res.AddRange(right.InorderTrav());
        }
        return res;
    }

    public static List<TreeNode> GenerateTrees(int n)
    {
        return Solve(1, n, []);

        static List<TreeNode> Solve(int start, int end, Dictionary<(int, int), List<TreeNode>> seen)
        {
            if (start > end)
            {
                return [null];
            }

            if (seen.TryGetValue((start, end), out var trees))
            {
                return trees;
            }

            List<TreeNode> res = [];
            for (int curr = start; curr <= end; curr++)
            {
                var leftTree = Solve(start, curr - 1, seen);
                var rightTree = Solve(curr + 1, end, seen);
                foreach (var left in leftTree)
                {
                    foreach (var right in rightTree)
                    {
                        TreeNode root = new(curr, left, right);
                        res.Add(root);
                    }
                }
            }
            seen.Add((start, end), res);
            return res;
        }

    }
}