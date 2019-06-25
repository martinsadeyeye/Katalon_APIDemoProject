<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ValidateRequest</name>
   <tag></tag>
   <elementGuidId>a1fbd8c1-edd6-4b35-b851-36e4d9ad29cb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;customFields\&quot;: \n\t[\n\t{\n\t\t\&quot;id\&quot;: \&quot;1509377917645\&quot;\n\t},\n\t\t{\n\t\t\&quot;id\&quot;: \&quot;1509377917653\&quot;\n\t},\n\t{\n\t\t\&quot;id\&quot;: \&quot;1509377917646\&quot;,\n\t\t\&quot;values\&quot;: \n\t\t[\n\t\t\t{\n\t\t\t\&quot;value\&quot;: \&quot;1509377917652\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0 \n\t\t\t\t\n\t\t\t},\n\t\t\t{\n\t\t\t\&quot;value\&quot;: \&quot;1509377917647\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0 \n\t\t\t\t\n\t\t\t},\n\t\t\t{\n\t\t\t\&quot;value\&quot;: \&quot;1509377917649\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0 \n\t\t\t},\n\t\t\t{\n\t\t\t\&quot;value\&quot;: \&quot;1509377917650\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0 \n\t\t\t\t\n\t\t\t},\n\t\t\t{\n\t\t\t\&quot;value\&quot;: \&quot;1509377917648\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0 \n\t\t\t\t\n\t\t\t},\n\t\t\t{\n\t\t\t\&quot;value\&quot;: \&quot;1509377917651\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0 \n\t\t\t\t\n\t\t\t}\n\t\t]\n\t}\n\t\n\t],\n\t\n\t\n\t\&quot;billId\&quot;: \&quot;1509377917654\&quot;,\n\t\&quot;billerId\&quot;: \&quot;C0000115401\&quot;,\n\t\&quot;amount\&quot;: 500,\n\t\&quot;payerPhone\&quot;: \&quot;07038496242\&quot;,\n\t\&quot;currency\&quot;: \&quot;NGN\&quot;,\n\t\&quot;payerName\&quot;: \&quot;DANIS IGWE \&quot;,\n\t\&quot;payerEmail\&quot;: \&quot;awoedey2k@gmail.com\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>publicKey</name>
      <type>Main</type>
      <value>U0hFTEx8MTUwOTM3MTg1NDczOXwyNjdjNDBiZmI5ZjMzMjg5M2I3MWI2YzEzZWUxYTQ5YjUxOTRhMjY5ZDljOWUzNmI0MWUxOTgyYzI1NDUyYTMxM2NlM2QxYTdmZjQxMTExN2M5MTU1NjgxNWYyYmEwMTI3ZWY3MmU4M2MxNmE2ZjBmNjE3Y2Q2OTNlYzA1ODA4Nw==</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>transactionId</name>
      <type>Main</type>
      <value></value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://192.9.200.209:6200/microservice/remita/gateway/send/api/bgatesvc/billing/validate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
