using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 4, 2, 3 }, new[] { 1, 2, 3, 4 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        var head = ListNode.Make(nums);
        sol.ReorderList(head);
        Assert.IsTrue(exp.SequenceEqual(head));
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