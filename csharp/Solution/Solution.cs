using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode MergeKLists(ListNode[] lists)
    {
        if (lists.Length == 0) { return null; }
        return Merge(lists, 0, lists.Length - 1);
    }

    static ListNode Merge(Span<ListNode> lists, int left, int right)
    {
        if (left == right) { return lists[left]; }
        int mid = left + (right - left) / 2;
        var a = Merge(lists, left, mid);
        var b = Merge(lists, 1 + mid, right);
        return Merge(a, b);
    }

    static ListNode Merge(ListNode a, ListNode b)
    {
        if (a is null) { return b; }
        if (b is null) { return a; }
        if (a.val <= b.val)
        {
            a.next = Merge(a.next, b);
            return a;
        }
        else
        {
            b.next = Merge(a, b.next);
            return b;
        }
    }
}
