using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinimumOperations(TreeNode root)
    {
        Queue<TreeNode> nodes = [];
        nodes.Enqueue(root);
        var res = 0;
        while (nodes.Count > 0)
        {
            var nums = nodes.Select(n => n.val).ToList();
            res += MinSwaps(nums);
            var n = nodes.Count;
            for (int i = 0; i < n; i++)
            {
                var node = nodes.Dequeue();
                if (node.left is not null) { nodes.Enqueue(node.left); }
                if (node.right is not null) { nodes.Enqueue(node.right); }
            }
        }
        return res;

        static int MinSwaps(IList<int> nums)
        {
            if (nums.Count < 2) { return 0; }
            var res = 0;
            var arr_pos = nums.Select((num, idx) => (idx, num)).OrderBy(p => p.num).ToList();
            var visisted = Enumerable.Range(0, nums.Count).Select(_ => false).ToList();
            for (int i = 0; i < nums.Count; i++)
            {
                if (visisted[i] || arr_pos[i].idx == i) { continue; }
                var cycle = 0;
                var curr = i;
                while (!visisted[curr])
                {
                    visisted[curr] = true;
                    curr = arr_pos[curr].idx;
                    cycle += 1;
                }
                res += Math.Max(0, cycle - 1); // only adds when cycle>1
            }
            return res;
        }
    }

    public int WithDictionary(TreeNode root)
    {
        Queue<TreeNode> queue = [];
        queue.Enqueue(root);
        var res = 0;
        while (queue.Count > 0)
        {
            var n = queue.Count;
            List<int> nums = [];
            for (int i = 0; i < n; i++)
            {
                var node = queue.Dequeue();
                nums.Add(node.val);
                if (node.left is not null) { queue.Enqueue(node.left); }
                if (node.right is not null) { queue.Enqueue(node.right); }
            }
            res += MinSwaps(nums);
        }
        return res;

        static int MinSwaps(IList<int> nums)
        {
            var target = nums.Order().ToList();
            var pos = nums.Select((n, i) => (n, i)).ToDictionary();
            var res = 0;
            for (int i = 0; i < nums.Count; i++)
            {
                if (nums[i] != target[i])
                {
                    res += 1;
                    var curr_pos = pos[target[i]];
                    pos[nums[i]] = curr_pos;
                    nums[curr_pos] = nums[i];
                }
            }
            return res;
        }
    }

    public int WithBits(TreeNode root)
    {
        const int SHIFT = 20;
        const int MASK = 0xFFFFF;
        Queue<TreeNode> queue = [];
        queue.Enqueue(root);
        var res = 0;
        while (queue.Count > 0)
        {
            var n = queue.Count;
            List<long> nums = [];
            for (int i = 0; i < n; i++)
            {
                var node = queue.Dequeue();
                nums.Add((node.val << SHIFT) + i); // value-index
                if (node.left is not null) { queue.Enqueue(node.left); }
                if (node.right is not null) { queue.Enqueue(node.right); }
            }
            nums.Sort();
            for (int i = 0; i < n; i++)
            {
                var original_pos = (int)(nums[i] & MASK);
                if (original_pos != i)
                {
                    (nums[i], nums[original_pos]) = (nums[original_pos], nums[i]);
                    i -= 1;
                    res += 1;
                }
            }
        }
        return res;
    }
}