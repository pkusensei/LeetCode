using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode ModifiedList(int[] nums, ListNode head)
    {
        HashSet<int> set = [.. nums];
        ListNode dummy = new(-1, head);
        var curr = dummy;
        while (curr.next is not null)
        {
            if (set.Contains(curr.next.val)) { curr.next = curr.next.next; }
            else { curr = curr.next; }
        }
        return dummy.next;
    }
}