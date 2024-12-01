using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 10, 2, 5, 3 }, true)]
    [DataRow(new int[] { 3, 1, 7, 11 }, false)]
    public void TestMethod1(int[] nums, bool exp)
    {
        Assert.AreEqual(exp, sol.CheckIfExist(nums));
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