using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = ListNode.Make([1, 2, 3]);
        var b = sol.SplitListToParts(a, 5);
        var c = "[[1],[2],[3],[],[]]";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = ListNode.Make([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        var b = sol.SplitListToParts(a, 3);
        var c = "[[1,2,3,4],[5,6,7],[8,9,10]]";
        Assert.AreEqual(c, b.Print());
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