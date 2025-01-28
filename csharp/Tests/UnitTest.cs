using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 10, 1, 13, 6, 9, 5 }, 3, 4, new[] { 1000000, 1000001, 1000002 }, "[10,1,13,1000000,1000001,1000002,5]")]
    [DataRow(new[] { 0, 1, 2, 3, 4, 5, 6 }, 2, 5, new[] { 1000000, 1000001, 1000002, 1000003, 1000004 }, "[0,1,1000000,1000001,1000002,1000003,1000004,6]")]
    public void TestMethod1(int[] n1, int a, int b, int[] n2, string exp)
    {
        var list1 = ListNode.Make(n1);
        var list2 = ListNode.Make(n2);
        Assert.AreEqual(exp, sol.MergeInBetween(list1, a, b, list2).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}