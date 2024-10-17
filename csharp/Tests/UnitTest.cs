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
        // var a = ListNode.Make([1, 2, 3]);
        var b = sol.MaximumSwap(2736);
        var c = 7236;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        // var a = ListNode.Make([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        var b = sol.MaximumSwap(9973);
        var c = 9973;
        Assert.AreEqual(c, b);
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