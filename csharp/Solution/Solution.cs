using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode MergeTwoLists(ListNode list1, ListNode list2)
    {
        ListNode dummy = new(0);
        var curr = dummy;
        while (list1 is not null && list2 is not null)
        {
            if (list1.val <= list2.val)
            {
                curr.next = list1;
                list1 = list1.next;
            }
            else
            {
                curr.next = list2;
                list2 = list2.next;
            }
            curr = curr.next;
        }
        if (list1 is not null) { curr.next = list1; }
        if (list2 is not null) { curr.next = list2; }
        return dummy.next;
    }
}
