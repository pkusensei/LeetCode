using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 0 }, new[] { 0, 0, 0 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.MajorityElement(nums)));
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