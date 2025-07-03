using System.Security.Principal;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
    {
        ListNode dummy = new(0);
        var n1 = l1;
        var n2 = l2;
        int carry = 0;
        var curr = dummy;
        while (n1 is not null || n2 is not null)
        {
            int val = carry + (n1 is null ? 0 : n1.val)
                            + (n2 is null ? 0 : n2.val);
            carry = val / 10;
            val %= 10;
            curr.next = new(val);
            curr = curr.next;
            n1 = n1?.next;
            n2 = n2?.next;
        }
        if (carry > 0) { curr.next = new(carry); }
        return dummy.next;
    }
}
