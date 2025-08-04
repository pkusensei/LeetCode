using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(ListNode head)
    {
        Rng = new();
        Head = head;
    }

    Random Rng { get; }
    ListNode Head { get; }

    public int GetRandom()
    {
        int res = 0;
        int i = 1;
        var curr = Head;
        while (curr is not null)
        {
            if (Rng.Next() % i == 0) { res = curr.val; }
            curr = curr.next;
            i += 1;
        }
        return res;
    }
}