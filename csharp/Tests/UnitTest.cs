using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(7, new[] { 1, 2, 3 }, 4)]
    public void TestMethod1(int exp, int[] nums, int t)
    {
        Assert.AreEqual(exp, sol.CombinationSum4(nums, t));
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