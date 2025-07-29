using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(true, "9,3,4,#,#,1,#,#,2,#,6,#,#")]
    [DataRow(false, "1,#")]
    [DataRow(false, "9,#,#,1")]
    public void TestMethod1(bool exp, string s)
    {
        Assert.AreEqual(exp, sol.IsValidSerialization(s));
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