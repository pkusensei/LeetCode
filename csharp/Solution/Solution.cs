using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string ClearStars(string s)
    {
        PriorityQueue<(int, char), (char, int)> pq = new();
        for (int i = 0; i < s.Length; i++)
        {
            if (s[i] == '*') { pq.Dequeue(); }
            else { pq.Enqueue((i, s[i]), (s[i], -i)); }
        }
        return new([.. pq.UnorderedItems.OrderBy(p => p.Element.Item1).Select(p => p.Element.Item2)]);
    }
}
