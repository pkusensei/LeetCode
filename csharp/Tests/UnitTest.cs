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
        var a = @"<DIV>This is the first line <![CDATA[<div>]]></DIV>";
        Assert.IsTrue(sol.IsValid(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = @"<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>";
        Assert.IsTrue(sol.IsValid(a));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = @"<A>  <B> </A>   </B>";
        Assert.IsFalse(sol.IsValid(a));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}