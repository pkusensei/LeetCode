using System.Diagnostics;
using System.Text;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var a = ListNode.Make([3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
    var b = SpiralMatrix(3, 5, a);
    var sb = new StringBuilder();
    sb.Append('[');
    sb.AppendJoin(',', b.Select(r => r.Print()));
    sb.Append(']');
    var c = "[[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]";
    Debug.Assert(sb.ToString() == c, $"Output: {sb}\nExpect: {c}");
}

void Test2()
{
    var a = ListNode.Make([0, 1, 2]);
    var b = SpiralMatrix(1, 4, a);
    var sb = new StringBuilder();
    sb.Append('[');
    sb.AppendJoin(',', b.Select(r => r.Print()));
    sb.Append(']');
    var c = "[[0,1,2,-1]]";
    Debug.Assert(sb.ToString() == c, $"Output: {sb}\nExpect: {c}");
}

int[][] SpiralMatrix(int m, int n, ListNode head)
{
    var res = new int[m][];
    for (int i = 0; i < m; i++)
    {
        res[i] = new int[n];
        Array.Fill(res[i], -1);
    }
    var (row, col) = (0, 0);
    var (row_min, col_min) = (0, 0);
    var done = false;
    while (!done)
    {
        for (col = col_min; col < n; col += 1)
        {
            res[row][col] = head.val;
            head = head.next;
            if (head is null) { done = true; break; }
        }
        col = n - 1;
        row_min += 1;
        if (done) { break; }
        for (row = row_min; row < m; row += 1)
        {
            res[row][col] = head.val;
            head = head.next;
            if (head is null) { done = true; break; }
        }
        row = m - 1;
        n -= 1;
        col -= 1;
        if (done) { break; }
        for (col = n - 1; col >= col_min; col -= 1)
        {
            res[row][col] = head.val;
            head = head.next;
            if (head is null) { done = true; break; }
        }
        col = col_min;
        m -= 1;
        if (done) { break; }
        for (row = m - 1; row >= row_min; row -= 1)
        {
            res[row][col] = head.val;
            head = head.next;
            if (head is null) { done = true; break; }
        }
        row = row_min;
        col_min += 1;
    }
    return res;
}
