using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode MergeInBetween(ListNode list1, int a, int b, ListNode list2)
    {
        var curr = list1;
        var count = 0;
        while (count < a - 1)
        {
            count += 1;
            curr = curr.next;
        }
        var prev = curr;
        while (count < b)
        {
            count += 1;
            curr = curr.next;
        }
        var last = curr.next;
        prev.next = list2;
        curr = list2;
        while (curr.next is not null)
        {
            curr = curr.next;
        }
        curr.next = last;
        return list1;
    }
}
