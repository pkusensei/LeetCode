using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3, 4 }, new[] { 4, 2, 1, 3 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        var head = ListNode.Make(nums);
        head = sol.SortList(head);
        Assert.AreEqual(exp.Print(), head.ToString());
        Assert.IsTrue(exp.SequenceEqual(sol.SortList(head)));
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