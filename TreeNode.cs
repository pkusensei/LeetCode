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
}