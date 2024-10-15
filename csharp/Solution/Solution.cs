using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode[] SplitListToParts(ListNode head, int k)
    {
        var count = 0;
        var curr = head;
        while (curr is not null)
        {
            count += 1;
            curr = curr.next;
        }
        var (ave, rem) = (count / k, count % k);
        var res = new ListNode[k];

        for (int i = 0; i < k; i++)
        {
            var n = rem > 0 ? ave + 1 : ave;
            rem -= 1;
            var (node, tail) = Split(head, n);
            res[i] = node;
            head = tail;
        }
        return res;
    }

    static (ListNode node, ListNode tail)
    Split(ListNode head, int n)
    {
        if (head is null) { return (null, null); }
        var curr = head;
        while (n > 1)
        {
            curr = curr.next;
            n -= 1;
        }
        var next = curr.next;
        curr.next = null;
        return (head, next);
    }
}
