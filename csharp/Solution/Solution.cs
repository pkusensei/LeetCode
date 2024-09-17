using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindMaximumXOR(int[] nums)
    {
        var trie = new Trie(nums);
        return trie.FindMaxXOR();
    }
}

internal class Node
{
    internal Node[] Children { get; set; }

    internal Node()
    {
        Children = new Node[2];
    }
}

internal class Trie
{
    internal Node Root { get; private set; }
    internal IList<int> Nums { get; private set; }
    internal Trie(IList<int> nums)
    {
        Root = new();
        Nums = nums;

        foreach (var num in Nums)
        {
            var node = Root;
            for (int bit = 31; bit >= 0; bit -= 1)
            {
                var currBit = (num >> bit) & 1;
                node.Children[currBit] ??= new();
                node = node.Children[currBit];
            }
        }
    }

    internal int FindMaxXOR()
    {
        var res = 0;
        foreach (var num in Nums)
        {
            var node = Root;
            var curr = 0;
            for (int bit = 31; bit >= 0; bit -= 1)
            {
                if (node is null) { break; }
                var seek = 1 - (num >> bit) & 1;
                if (node.Children[seek] is not null)
                {
                    curr |= (1 << bit);
                    node = node.Children[seek];
                }
                else
                {
                    node = node.Children[1 - seek];
                }
            }
            res = Math.Max(res, curr);
        }
        return res;
    }
}