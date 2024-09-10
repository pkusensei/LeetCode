using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode InsertGreatestCommonDivisors(ListNode head)
    {
        var curr = head;
        while (curr.next is not null)
        {
            var temp = curr.next;
            var gcd = GCD(curr.val, temp.val);
            ListNode node = new(gcd, temp);
            curr.next = node;
            curr = temp;
        }
        return head;
    }

    static int GCD(int a, int b)
    {
        if (a == 0) { return b; }
        else { return GCD(b % a, a); }
    }
}
